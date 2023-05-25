require File.join(__dir__, 'RustSketchupTest.so')

module RustExtension
  unless file_loaded?(__FILE__)

    # Polyhedron setup

    polyhedron_next_position = Geom::Point3d.new()

    # Terrain setup

    class TerrainTool
      def onLButtonUp(flags, x, y, view)
        ray = view.pickray(x, y)
        ground = [Geom::Point3d.new(0, 0 ,0), Geom::Vector3d.new(0, 0, 1)]
        point = Geom.intersect_line_plane(ray, ground)

        triangles = RustExtension::terrain_generate(100)

        Sketchup.active_model.start_operation('Create terrain', true)

        terrain = Sketchup.active_model.entities.add_group
        terrain.transform!(Geom::Transformation.translation(point.to_a) * Geom::Transformation.scaling(10))

        terrain.entities.build { |builder|
          triangles.each { |triangle|
            builder.add_face(*triangle)
          }
        }

        Sketchup.active_model.commit_operation

      end
    end

    # Physics setup

    all_materials = Sketchup.active_model.materials

    static_material = all_materials['physics static'] || all_materials.add('physics static')
    static_material.color = Sketchup::Color.new('Gray')

    dynamic_material = all_materials['physics dynamic'] || all_materials.add('physics dynamic')
    dynamic_material.color = Sketchup::Color.new('HotPink')

    prepare_objects = lambda do |entities, static|
      # Gather the entities' data

      data = entities.map do |entity|
        [
          # ID
          entity.persistent_id,
          # Center
          entity.bounds.center.to_a,
          # Size
          [entity.bounds.width, entity.bounds.height, entity.bounds.depth]
        ]
      end

      # Apply the appropriate material

      material = static ? static_material : dynamic_material

      Sketchup.active_model.entities.each do |entity|
        entity.material = nil if entity.material == material
      end

      entities.each do |entity|
        entity.material = material
      end

      data
    end

    static_initial = []
    dynamic_initial = []

    Sketchup.active_model.entities.each do |entity|
      if entity.material == static_material
        static_initial.push(entity)
      elsif entity.material == dynamic_material
        dynamic_initial.push(entity)
      end
    end

    physics_set_static_objects(prepare_objects.call(static_initial, true))
    physics_set_dynamic_objects(prepare_objects.call(dynamic_initial, false))

    # Game Boy setup

    class GameBoyTool
      def activate
        rom_path = UI.openpanel("Open Game Boy ROM", "", "Game Boy ROMs|*.gb")

        puts "Starting Game Boy with #{rom_path}"

        RustExtension::gameboy_load_rom(rom_path)

        timer = UI.start_timer(1.0 / 60.0, true) do
          screen_buffer = RustExtension::gameboy_run_frame(1)

          image = Sketchup::ImageRep.new
          image.set_data(160, 144, 24, 0, screen_buffer.pack("C*"))

          screen_material = Sketchup.active_model.materials['screen'] || Sketchup.active_model.materials.add("screen")
          screen_material.texture = image

          Sketchup.active_model.active_view.invalidate
        end
      end

      def get_button_name(key)
        names = {
          37 => 'left',
          38 => 'up',
          39 => 'right',
          40 => 'down',
          97 => 'a',
          98 => 'b',
          100 => 'start',
          101 => 'select',
        }

        names[key]
      end

      def onKeyDown(key, repeat, flags, view)
        button = get_button_name(key)

        if button
          RustExtension::gameboy_press_button(button)
          return true
        else
          return false
        end
      end

      def onKeyUp(key, repeat, flags, view)
        button = get_button_name(key)

        if button
          RustExtension::gameboy_release_button(button)
          return true
        else
          return false
        end
      end
    end

    class GameboySelectionObserver < Sketchup::SelectionObserver
      def onSelectionBulkChange(selection)
        gameboy = Sketchup.active_model.entities.find { |e| e.is_a?(Sketchup::ComponentInstance) && e.name == 'game boy' }

        if selection.contains?(gameboy)
          Sketchup.active_model.select_tool(GameBoyTool.new)
        end
      end
    end

    Sketchup.active_model.selection.add_observer(GameboySelectionObserver.new)

    # Menu

    UI.add_context_menu_handler do |menu|

      # Polyhedron

      menu.add_item("Create random polyhedron") {
        polyhedron = generate_polyhedron

        model = Sketchup.active_model

        model.start_operation('Create polyhedron', true)

        group = model.entities.add_group

        group.entities.build { |builder|
          polyhedron.each do |face|
            builder.add_face(face)
          end
        }

        scale = Geom::Transformation.scaling(10)

        translation = Geom::Transformation.translation(polyhedron_next_position)
        polyhedron_next_position += Geom::Vector3d.new(25, 0, 0)

        group.transform!(translation * scale)

        model.commit_operation
      }

      menu.add_separator

      # Terrain

      menu.add_item("Terrain: generate") {
        Sketchup.active_model.select_tool(TerrainTool.new)
      }

      menu.add_separator

      # Physics

      menu.add_item("Physics: set static") {
        data = prepare_objects.call(Sketchup.active_model.selection.to_a, true)
        physics_set_static_objects(data)
      }

      menu.add_item("Physics: set dynamic") {
        data = prepare_objects.call(Sketchup.active_model.selection.to_a, false)
        physics_set_dynamic_objects(data)
      }

      menu.add_item("Physics: simulate") {
        puts "Starting physics simulation"

        frames = physics_simulate(200)

        timer = UI.start_timer(1.0 / 60.0, true) do
          frame = frames.shift

          if frame.nil?
            puts "Stopping physics simulation"
            UI.stop_timer(timer)
          else
            frame.each do |object_data|
              id = object_data[0]
              entity = Sketchup.active_model.find_entity_by_persistent_id(id)

              scale = Geom::Transformation.scaling(
                Geom::Vector3d.new(entity.transformation.to_a[0..2]).length,
                Geom::Vector3d.new(entity.transformation.to_a[4..6]).length,
                Geom::Vector3d.new(entity.transformation.to_a[8..10]).length
              )

              translation = Geom::Transformation.translation(object_data[1])

              rotation = Geom::Transformation.rotation(
                Geom::Point3d.new(0, 0, 0),
                object_data[2].slice(0, 3),
                object_data[2][3]
              )

              entity.move!(translation * rotation * scale)
            end

            Sketchup.active_model.active_view.invalidate
          end
        end
      }

      menu.add_separator
    end
  end
end

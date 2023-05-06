require File.join(__dir__, 'RustSketchupTest.so')

module RustTest
  unless file_loaded?(__FILE__)

    UI.add_context_menu_handler do |menu|

      # Polyhedron

      menu.add_item("Polyhedron") {
        polyhedron = RustTest::Rust.generate_polyhedron

        model = Sketchup.active_model

        model.start_operation('Create polyhedron', true)

        group = model.entities.add_group

        group.entities.build { |builder|
          polyhedron.each do |face|
            builder.add_face(face)
          end
        }

        model.commit_operation
      }

      # Physics

      all_materials = Sketchup.active_model.materials

      static_material = all_materials['physics static'] || all_materials.add("physics static")
      static_material.color = 'red'

      dynamic_material = all_materials['physics dynamic'] || all_materials.add("physics dynamic")
      dynamic_material.color = 'green'

      prepare_objects = lambda do |entities, static|

        # Gather the entities' data

        data = entities.map do |entity|
          triangles = []

          faces = entity.entities.select do |subentity|
            subentity.is_a?(Sketchup::Face)
          end

          faces.each do |face|
            face.mesh.polygons.each do |triangle|
              vertices = triangle.map { |i| face.mesh.point_at(i.abs).to_a }
              triangles.push(vertices)
            end
          end

          [
            # ID
            entity.persistent_id,
            # Transformation
            entity.transformation.origin.to_a,
            # Geometry
            triangles
          ]
        end

        # Apply a material

        material = static ? static_material : dynamic_material

        Sketchup.active_model.entities.each do |entity|
          entity.material = nil if entity.material == material
        end

        entities.each do |entity|
          entity.material = material
        end

        data
      end

      menu.add_item("Physics: set static") {
        data = prepare_objects.call(Sketchup.active_model.selection.to_a, true)
        RustTest::Rust.set_static_objects(data)
      }

      menu.add_item("Physics: set dynamic") {
        data = prepare_objects.call(Sketchup.active_model.selection.to_a, false)
        RustTest::Rust.set_dynamic_objects(data)
      }

      menu.add_item("Physics: simulate") {
        simulation = RustTest::Rust.simulate(200)

        frame_index = 0

        timer = UI.start_timer(1.0 / 60.0, true) do
          frame = simulation[frame_index]

          frame.each do |object_data|
            id = object_data[0]
            entity = Sketchup.active_model.find_entity_by_persistent_id(id)

            position = object_data[1]
            entity.move!(Geom::Transformation.translation(position))
          end

          Sketchup.active_model.active_view.invalidate

          frame_index += 1

          UI.stop_timer(timer) if frame_index >= simulation.length()
        end
      }
    end

  end
end

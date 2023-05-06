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

      menu.add_item("Physics") {
        selection = Sketchup.active_model.selection.to_a

        data = selection.map do |entity|
          entity.transformation.origin.to_a
        end

        simulation = RustTest::Rust.simulate(data)
        frame_index = 0

        timer = UI.start_timer(1.0 / 60.0, true) do
          frame = simulation[frame_index]

          selection.each_with_index do |entity, entity_index|
            position = frame[entity_index]
            entity.move!(Geom::Transformation.translation(position))
          end

          Sketchup.active_model.active_view.invalidate

          frame_index += 1

          UI.stop_timer(timer) if frame_index >= simulation.length()
        end

        # model = Sketchup.active_model

        # model.start_operation('Create polyhedron', true)

        # group = model.entities.add_group

        # group.entities.build { |builder|
        #   polyhedron.each do |face|
        #     builder.add_face(face)
        #   end
        # }

        # model.commit_operation
      }
    end

  end
end

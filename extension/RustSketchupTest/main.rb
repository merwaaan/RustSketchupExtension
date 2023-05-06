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
    end

  end
end

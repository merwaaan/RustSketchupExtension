require File.join(__dir__, 'RustSketchupTest.so')

module RustTest
  unless file_loaded?(__FILE__)

    # Setup the UI

    test_cmd = UI::Command.new('Import model...') do
      puts 'Calling Rust function...'
      # TODO
    end

    test_cmd.tooltip = 'test'
    test_cmd.menu_text = 'test'

    # Build the menu

    transmutr_menu = UI.menu('Plugins').add_submenu('RustSketchupTest')

    transmutr_menu.add_item(test_cmd)
  end
end

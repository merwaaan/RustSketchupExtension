require 'sketchup'
require 'extensions'

module RustExtension
  unless file_loaded?(__FILE__)

    # Register the extension

    ext = SketchupExtension.new('RustSketchupTest', 'RustSketchupTest/main')
    ext.version = '1.0.0'
    ext.creator = 'Lindalë'
    ext.copyright = "Lindalë SARL © #{Time.new.year}"
    ext.description = 'Prototype for a SketchUp extension in Rust'

    Sketchup.register_extension(ext, true)

    file_loaded(__FILE__)
  end
end

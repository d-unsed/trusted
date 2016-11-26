require 'concurrent'
require 'docile'
require 'thermite/fiddle'

require 'trusted/config/builder'
require 'trusted/config/config'

require 'trusted/request/processing_pool'

require 'trusted/version'

require 'rack/handler/trusted'

toplevel_dir = File.dirname(File.dirname(__FILE__))

Thermite::Fiddle.load_module(
  'initialize_my_app',
  ruby_project_path: toplevel_dir,
  cargo_project_path: File.join(toplevel_dir, 'ext', 'trusted')
)

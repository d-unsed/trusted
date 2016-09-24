require 'fiddle'

require 'trusted/request'
require 'trusted/response'
require 'trusted/version'

require 'rack/handler/trusted'

library_path = File.expand_path(
  File.join(File.dirname(__FILE__), 'libtrusted.dylib')
)

library = Fiddle::dlopen(library_path)
function = Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP)
function.call

module Trusted
end

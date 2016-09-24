require 'trusted/version'

require 'rack/handler/trusted'
require 'fiddle'

library_path = File.expand_path(
  File.join(File.dirname(__FILE__), 'libtrusted.dylib')
)

library = Fiddle::dlopen(library_path)
function = Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP)
function.call

module Trusted
  # Your code goes here...
end

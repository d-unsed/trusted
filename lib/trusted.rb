require 'trusted/version'

require 'fiddle'

library_path = File.expand_path(
  File.join(File.dirname(__FILE__), '../ext/trusted/target/release/libtrusted.dylib')
)

library = Fiddle::dlopen(library_gpath)
function = Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP)
function.call

module Trusted
  # Your code goes here...
end

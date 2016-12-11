module Trusted
  module Config
    class Config
      attr_reader :binding_type, :listen_on, :rack_thread_pool_size, :native_thread_pool_size

      def initialize(configuration)
        @binding_type = configuration[:binding_type]
        @listen_on = configuration[:listen_on]
        @rack_thread_pool_size = configuration[:rack_thread_pool_size]
        @native_thread_pool_size = configuration[:native_thread_pool_size]
      end
    end
  end
end

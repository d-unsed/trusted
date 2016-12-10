module Trusted
  module Config
    class Config
      attr_reader :binding_type, :listen_on, :thread_pool_size

      def initialize(configuration)
        @binding_type = configuration[:binding_type]
        @listen_on = configuration[:listen_on]
        @thread_pool_size = configuration[:thread_pool_size]
      end
    end
  end
end

module Trusted
  module Config
    class Builder
      attr_reader :config

      DEFAULT_CONFIG = {
        binding_type: :tcp,
        listen_on: 'localhost:3000',
      }.freeze

      def initialize
        @config = {}
      end

      def binding_type(type)
        config[:binding_type] = type
      end

      def listen_on(address)
        config[:listen_on] = address
      end

      def build
        config = DEFAULT_CONFIG.merge(config)
      end
    end
  end
end

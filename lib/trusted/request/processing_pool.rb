module Trusted
  module Request
    class ProcessingPool
      attr_reader :handler, :thread_pool

      def initialize(handler)
        @handler = handler
        @thread_pool = Concurrent::FixedThreadPool.new(5)
      end

      private

      def execute_future(request, response, observer)
        future = Concurrent::Future.new(executor: thread_pool, args: [request, response], &handler)

        future.add_observer(observer)
        future.execute
      end
    end
  end
end

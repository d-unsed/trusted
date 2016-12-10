module Trusted
  module Request
    class ProcessingPool
      attr_reader :handler, :thread_pool

      def initialize(handler, thread_pool_size)
        @handler = handler
        @thread_pool = Concurrent::FixedThreadPool.new(thread_pool_size)

        puts "[ruby] Spawning #{thread_pool_size} green thread(s)"
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

require 'rack'
require 'rack/request'
require 'stringio'

module Rack
  module Handler
    class Trusted
      def self.run(app, options = {})
        config = options[:trusted_config] || ::Trusted::Config::Builder.new.build

        ::Trusted::Server.new(config).listen do |request, response|
          puts "REQUEST: [#{request.method}] #{request.uri}"
          puts "PATH_INFO: #{request.path_info}"
          puts "QUERY_STRING: #{request.query_string}"
          puts "REMOTE_ADDR: #{request.remote_addr.inspect}"
          puts "SERVER_PORT: #{request.server_port.inspect}"
          puts "REQUEST HEADERS: #{request.headers.inspect}"

          rack_input = StringIO::new(request.body)

          env = {
            'REQUEST_METHOD' => request.method,
            'REQUEST_URI' => request.uri,
            'PATH_INFO' => request.path_info,
            'QUERY_STRING' => request.query_string,
            'REMOTE_ADDR' => request.remote_addr,
            'SERVER_PORT' => '3000',
            'SERVER_NAME' => 'localhost',
            'SCRIPT_NAME' => '',
            'rack.version' => Rack::VERSION,
            'rack.input' => rack_input,
            'rack.errors' => $stderr,
            'rack.multithread' => false,
            'rack.multiprocess' => false,
            'rack.run_once' => false,
            'rack.url_scheme' => 'http',
            'rack.hijack?' => false
          }

          env.merge!(request.headers)

          status, headers, body = app.call(env)

          response.status = status
          response.headers = headers

          response.body = ''

          body.each { |b| response.body.concat(b) }

          body.close if body.respond_to?(:close)
        end
      end
    end

    register :trusted, 'Rack::Handler::Trusted'

    def self.default(options = {})
      Rack::Handler::Trusted
    end
  end
end

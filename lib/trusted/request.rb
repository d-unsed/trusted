module Trusted
  class Request
    attr_reader :method, :uri, :path_info, :query_string, :remote_addr, :server_port, :headers, :body

    def initialize(method, uri, path_info, query_string, remote_addr, server_port, headers, body)
      @method = method
      @uri = uri
      @path_info = path_info
      @query_string = query_string
      @remote_addr = remote_addr
      @server_port = server_port
      @headers = headers
      @body = body
    end
  end
end

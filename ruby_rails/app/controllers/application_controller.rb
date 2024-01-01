class ApplicationController < ActionController::API
  def hello_world
    render plain: 'Hello World'
  end
end

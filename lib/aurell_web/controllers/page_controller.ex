defmodule AurellWeb.PageController do
  use AurellWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end

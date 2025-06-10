defmodule AurellWeb.ErrorJSONTest do
  use AurellWeb.ConnCase, async: true

  test "renders 404" do
    assert AurellWeb.ErrorJSON.render("404.json", %{}) == %{errors: %{detail: "Not Found"}}
  end

  test "renders 500" do
    assert AurellWeb.ErrorJSON.render("500.json", %{}) ==
             %{errors: %{detail: "Internal Server Error"}}
  end
end

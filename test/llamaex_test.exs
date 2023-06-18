defmodule LlamaexTest do
  use ExUnit.Case
  doctest Llamaex

  test "greets the world" do
    assert Llamaex.hello() == :world
  end
end

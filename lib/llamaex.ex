defmodule Llamaex do
  def adder(arg1, arg2) do
    Llamaex.Native.add(arg1, arg2)
  end
end

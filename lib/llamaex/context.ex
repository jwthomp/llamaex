defmodule Llamaex.Context do
  @enforce_keys [:reference]
  defstruct [:reference]
end

defimpl Inspect, for: Llamaex.Context do
  import Inspect.Algebra

  def inspect(%Llamaex.Context{reference: ctx}, opts) do
    concat(["#Llamaex.Context<", to_doc(ctx, opts), ">"])
  end
end

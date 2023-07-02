defmodule Llamaex do
  def add(arg1, arg2) do
    Llamaex.Native.add(arg1, arg2)
  end

  def llama_init_from_file(filename) when is_binary(filename) do
    context = Llamaex.Native.llama_init_from_file(filename)
    {:ok, context}
  end

  def llama_tokenize(context, prompt) do
    result = Llamaex.Native.llama_tokenize(context, prompt)
    {:ok, result}
  end

  def llama_free(context) do
    Llamaex.Native.llama_free(context)
    :ok
  end
end



defmodule Llamaex.Native do
  @moduledoc false

  use Rustler,
    otp_app: :llamaex,
    crate: :llamaex

  def add(_a, _b), do: error()

  def llama_init_from_file(_filename), do: error()
  def llama_tokenize(_ctx, _prompt), do: error()
  def llama_free(_ctx), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)



end

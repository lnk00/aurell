defmodule Aurell.Repo do
  use Ecto.Repo,
    otp_app: :aurell,
    adapter: Ecto.Adapters.Postgres
end

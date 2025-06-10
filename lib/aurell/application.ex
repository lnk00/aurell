defmodule Aurell.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      AurellWeb.Telemetry,
      Aurell.Repo,
      {DNSCluster, query: Application.get_env(:aurell, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Aurell.PubSub},
      # Start a worker by calling: Aurell.Worker.start_link(arg)
      # {Aurell.Worker, arg},
      # Start to serve requests, typically the last entry
      AurellWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Aurell.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    AurellWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end

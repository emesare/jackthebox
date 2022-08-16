using Sandbox;

namespace ExampleAddon;

public class Addon
{
    // Entry point for your addons.
    internal static void Main()
    {
        Log.Info("Call from an addon that can be loaded into dedicated servers and clients");
    }
}
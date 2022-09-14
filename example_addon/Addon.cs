using Sandbox;

static public class Addon
{
    // Entry point for your addons.
    public static void OnLoad()
    {
        Log.Info($"This addon is executing in the host named {Host.Name}");
    }

    [ConCmd.Client("custom_thirdperson")]
    public static void ThirdPersonCmd()
    {
        Log.Info($"pawn:lol {Local.Pawn}");
        Log.Info($"client: {Local.Client}");
        // if (Local.Client is Player player)
        // {
        //     Log.Info("Oh no.");
        // }
    }
}
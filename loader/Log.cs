using System.Reflection;

// Util class to remove dependence on the `Sandbox.System` assembly.
static class Log
{
    public static void Info(System.FormattableString message)
    {
        System.Type? type = System.Type.GetType("Sandbox.Logger, Sandbox.System");
        type?.GetRuntimeMethod("Info", new System.Type[] { typeof(System.FormattableString) })?.Invoke(GetGlobalLoggerInstance(), new object[] { message });
    }

    public static void Warning(System.FormattableString message)
    {
        System.Type? type = System.Type.GetType("Sandbox.Logger, Sandbox.System");
        type?.GetRuntimeMethod("Warning", new System.Type[] { typeof(System.FormattableString) })?.Invoke(GetGlobalLoggerInstance(), new object[] { message });
    }

    public static void Error(System.FormattableString message)
    {
        System.Type? type = System.Type.GetType("Sandbox.Logger, Sandbox.System");
        type?.GetRuntimeMethod("Error", new System.Type[] { typeof(System.FormattableString) })?.Invoke(GetGlobalLoggerInstance(), new object[] { message });
    }

    static object? GetGlobalLoggerInstance()
    {
        System.Type? type = System.Type.GetType("Sandbox.Internal.GlobalGameNamespace, Sandbox.Game");
        return type?.GetRuntimeProperty("Log")?.GetValue(null);
    }
}
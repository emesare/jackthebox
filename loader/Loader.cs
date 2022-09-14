using System;
using System.IO;
using System.Runtime.Loader;
using System.Reflection;
using System.Collections.Generic;

namespace Jackload;

public class Loader
{
    static List<RealmWatcher> watchers = new();

    internal static void Main()
    {
        Log.Info($"JackTheBox has been loaded!");
        foreach (var ctx in AssemblyLoadContext.All)
        {
            if (ctx.GetType().Name == "SandboxedLoadContext")
            {
                using (ctx.EnterContextualReflection())
                {
                    // Now use reflection API to get the realm name in `Host`.
                    Type? hostTy = Type.GetType("Sandbox.Host, Sandbox.Game");
                    if (hostTy == null) continue;
                    PropertyInfo? nameProp = hostTy.GetRuntimeProperty("Name");
                    if (nameProp == null) continue;
                    string? name = (string?)nameProp.GetValue(null);
                    if (name == null) continue;
                    // Realm found, create a watcher!
                    DirectoryInfo realmDir = new DirectoryInfo(AppDomain.CurrentDomain.SetupInformation.ApplicationBase + @"sideload\" + $"{name}\\");
                    Log.Info($"Realm {name.ToUpper()} found, creating watcher at {realmDir.FullName}");
                    watchers.Add(new RealmWatcher(name, realmDir, ctx));
                }
            }
        }
    }
}
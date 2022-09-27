using System.IO;
using System.Runtime.Loader;
using System.Reflection;
using System.Collections.Generic;

// Watches a directory and loads all assemblies in it to a sandbox realm.
class RealmWatcher : FileSystemWatcher
{
    string name { get; }
    DirectoryInfo dir { get; }
    AssemblyLoadContext context { get; }
    Dictionary<FileInfo, ContextAssemblyLoader> loaders { get; set; }

    public RealmWatcher(string name, DirectoryInfo dir, AssemblyLoadContext context) : base(dir.FullName, "*.dll")
    {
        this.name = name;
        this.context = context;
        this.dir = dir;
        this.loaders = new();

        // Load all assemblies in the dir path.
        foreach (FileInfo dllFile in dir.GetFiles("*.dll"))
        {
            Log.Info($"Attempting to load assembly {dllFile.Name}...");
            if (AddAssembly(dllFile) == null)
            {
                Log.Error($"Failed to load assembly {dllFile.Name}!");
            }
        }

        // Watch for new dlls to create context loaders for.
        // Support reloading the assembly, s&box will take care of hotloading.
        this.EnableRaisingEvents = true;
        this.Created += OnCreatedAssembly;
        this.Changed += OnUpdatedAssembly;
    }

    private void OnCreatedAssembly(object sender, FileSystemEventArgs e)
    {
        RealmWatcher? realm = sender as RealmWatcher;
        Log.Info($"Attempting to load new assembly {e.Name}...");
        if (realm?.AddAssembly(new FileInfo(e.FullPath)) == null)
        {
            Log.Error($"Failed to load new assembly {e.Name}!");
        }
    }

    private void OnUpdatedAssembly(object sender, FileSystemEventArgs e)
    {
        try
        {
            this.EnableRaisingEvents = false;
            RealmWatcher? realm = sender as RealmWatcher;
            Log.Info($"Attempting to reload assembly {e.Name}...");
            if (realm?.ReloadAssembly(new FileInfo(e.FullPath)) == true)
            {
                Log.Error($"Failed to reload assembly {e.Name}!");
            }
        }
        finally
        {
            this.EnableRaisingEvents = true;
        }
    }

    private Assembly? AddAssembly(FileInfo file)
    {
        try
        {
            using (Stream stream = file.OpenRead())
            {
                ContextAssemblyLoader loader = new ContextAssemblyLoader(context, file.Name, stream);
                Assembly? assembly = loader.LoadAssembly();
                if (assembly != null)
                {
                    loaders.TryAdd(file, loader);
                    ExecuteClassMethod(assembly, "Addon", "OnLoad");
                }
                return assembly;
            }
        }
        catch (System.Exception e)
        {
            Log.Error($"Exception occurred whilst adding assembly: {e}");
            return null;
        }
    }

    private bool ReloadAssembly(FileInfo file)
    {
        try
        {
            ContextAssemblyLoader? loader = null;
            if (loaders.TryGetValue(file, out loader) == true)
            {
                using (Stream stream = file.OpenRead())
                {
                    loader.assemblyStream = stream;
                    loader.RemoveAssembly();
                    Assembly? assembly = loader.LoadAssembly();
                    if (assembly != null)
                    {
                        ExecuteClassMethod(assembly, "Addon", "OnReload");
                    }
                    return true;
                }
            }

            return false;
        }
        catch (System.Exception e)
        {
            Log.Error($"Exception occurred whilst reloading assembly: {e}");
            return false;
        }
    }

    public static void ExecuteClassMethod(Assembly assembly, string className, string methodName)
    {
        string? assemblyName = assembly.GetName().Name;
        System.Type? type = assembly.GetType(className);
        if (type == null)
        {
            Log.Error($"Class {className} does not exist in assembly {assemblyName}!");
            return;
        }

        // Create a new instance of the class.
        object? classInstance = null;
        try
        {
            classInstance = System.Activator.CreateInstance(type, null);
        }
        catch (System.MissingMethodException) {/* This is fine, its a static class! */}

        // Get the entry point method.
        MethodInfo? methodInfo = type.GetMethod(methodName, BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic);
        if (methodInfo == null)
        {
            // Because sometimes you DONT need to have a method, it is optional (OnUpdate).
            Log.Warning($"Failed to find method {assemblyName}.{className}.{methodName}.");
            return;
        }

        // Checks params then call the entry point method.
        ParameterInfo[] parameters = methodInfo.GetParameters();
        if (parameters.Length == 0)
        {
            methodInfo.Invoke(classInstance, null);
        }
        else
        {
            Log.Error($"{assemblyName}.{className}.{methodName} cannot have any parameters!");
        }
    }
}
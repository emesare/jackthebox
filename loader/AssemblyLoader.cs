using System.IO;
using System.Reflection;
using System.Collections.Generic;

class AssemblyLoader
{
    public string name { get; }
    public Stream assemblyStream { get; set; }
    private Assembly? currentAssembly { get; set; }

    public AssemblyLoader(string name, Stream assemblyStream)
    {
        this.name = name;
        this.assemblyStream = assemblyStream;
        this.currentAssembly = null;
    }

    // Load assembly from disk, hotload if necessary, then return the loaded assembly.
    protected Assembly? LoadAssembly()
    {
        object? tbs = NewTrustedBinaryStream(assemblyStream);
        if (tbs == null) return null;
        object? options = NewAssemblyRegistration(name, tbs);
        if (options == null) return null;

        if (AddAssembly(options))
        {
            // Get the loaded assembly.
            foreach (FieldInfo rtfield in GetTypeLibraryType()?.GetRuntimeFields())
            {
                if (rtfield.Name == "loadedAssemblies")
                {
                    Dictionary<string, Assembly>? loadedAssemblies = (Dictionary<string, Assembly>?)rtfield.GetValue(GetGlobalTypeLibraryInstance());
                    if (loadedAssemblies == null)
                    {
                        Log.Error($"Failed to get loadedAssemblies value!");
                        break;
                    }
                    this.currentAssembly = loadedAssemblies[name];
                    return loadedAssemblies[name];
                }
            }
        }
        else
        {
            Log.Error($"Failed to invoke AddAssembly!");
        }

        return null;
    }

    protected void RemoveAssembly()
    {
        if (this.currentAssembly == null) return;

        MethodInfo? removeAssembly = GetTypeLibraryType()?.GetRuntimeMethod("RemoveAssembly", new System.Type[] { typeof(Assembly) });

        if (removeAssembly == null)
        {
            Log.Error($"Failed to get method RemoveAssembly!");
            return;
        }

        object? tlInstance = GetGlobalTypeLibraryInstance();
        if (tlInstance == null)
        {
            Log.Error($"Failed to get TypeLibrary instance!");
            return;
        }

        removeAssembly.Invoke(tlInstance, new object[] { this.currentAssembly });
    }

    private static bool AddAssembly(object assemblyRegistrationOptions)
    {
        // Find `AddAssembly`.
        MethodInfo? addAssembly = null;
        foreach (MethodInfo method in GetTypeLibraryType()?.GetRuntimeMethods())
        {
            // Make sure we dont get `internal void AddAssembly(Assembly incoming, bool isDynamic)`.
            if (method.Name == "AddAssembly" && method.ReturnType == typeof(bool))
            {
                addAssembly = method;
            }
        }

        if (addAssembly == null)
        {
            Log.Error($"Failed to find AddAssembly!");
            return false;
        };

        object? tlInstance = GetGlobalTypeLibraryInstance();
        if (tlInstance == null)
        {
            Log.Error($"Failed to get TypeLibrary instance!");
            return false;
        }

        return (bool?)addAssembly.Invoke(tlInstance, new object[] { assemblyRegistrationOptions }) == true;
    }

    private static object? NewAssemblyRegistration(string name, object trustedBinaryStream)
    {
        System.Type? type = System.Type.GetType("Sandbox.AssemblyRegistration, Sandbox.Reflection");
        if (type != null)
        {
            // Create a new instance of the class.
            object? registrationInstance = System.Activator.CreateInstance(type);
            // Set the instances values.
            registrationInstance?.GetType().GetMethod("set_Name")?.Invoke(registrationInstance, new object[] { name });
            registrationInstance?.GetType().GetMethod("set_DllStream")?.Invoke(registrationInstance, new object[] { trustedBinaryStream });
            return registrationInstance;
        }
        else
        {
            Log.Error($"Failed to find AssemblyRegistration!");
        }

        return null;
    }

    private static object? NewTrustedBinaryStream(Stream dllStream)
    {
        System.Type? type = System.Type.GetType("Sandbox.TrustedBinaryStream, Sandbox.Access");
        if (type != null)
        {
            // Create a new instance of the class with `CreateInternal`.
            return type.GetMethod("CreateInternal", BindingFlags.Static | BindingFlags.NonPublic)?.Invoke(null, new object[] { dllStream });
        }
        else
        {
            Log.Error($"Failed to find TrustedBinaryStream!");
        }

        return null;
    }

    private static object? GetGlobalTypeLibraryInstance()
    {
        System.Type? type = System.Type.GetType("Sandbox.Internal.GlobalGameNamespace, Sandbox.Game");
        return type?.GetRuntimeProperty("TypeLibrary")?.GetValue(null);
    }

    private static System.Type? GetTypeLibraryType()
    {
        return System.Type.GetType("Sandbox.Internal.TypeLibrary, Sandbox.Reflection");
    }
}
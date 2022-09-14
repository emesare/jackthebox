using System.Runtime.Loader;
using System.Reflection;
using System.IO;

class ContextAssemblyLoader : AssemblyLoader
{
    public AssemblyLoadContext context { get; }

    public ContextAssemblyLoader(AssemblyLoadContext context, string name, Stream assemblyStream) : base(name, assemblyStream)
    {
        this.context = context;
    }

    public new Assembly? LoadAssembly()
    {
        using (context.EnterContextualReflection())
        {
            return base.LoadAssembly();
        }
    }

    public new void RemoveAssembly()
    {
        using (context.EnterContextualReflection())
        {
            base.RemoveAssembly();
        }
    }
}
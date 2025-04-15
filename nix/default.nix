{
  imports = builtins.readDir ./modules |> builtins.attrNames |> map (name: ./modules/${name});
}

with import <nixpkgs> {};
with pkgs.python3Packages;

buildPythonPackage rec {
  name = "basedcast";
  src = ./api.py;
  #src = (fetchFromGitHub())
  propagatedBuildInputs = [ pytest numpy pkgs.libsndfile ];
}
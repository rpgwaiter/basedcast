with import <nixpkgs> {};
with pkgs.python3Packages;

buildPythonPackage rec {
  name = "basedcast";
  src = ./api.py;
  #src = (fetchFromGitHub())
  propagatedBuildInputs = [ mpd2 flask pkgs.libsndfile ];
}
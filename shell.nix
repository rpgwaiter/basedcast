with import <nixpkgs> {};
with pkgs.python37Packages;

buildPythonPackage rec {
  name = "basedcast";
  src = ./api.py;
  #src = (fetchFromGitHub())
  propagatedBuildInputs = [ mpd flask pkgs.libsndfile ];
}
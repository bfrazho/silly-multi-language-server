if not exist %HOMEDRIVE%%HOMEPATH%\solipath\solipath_setup.bat curl --create-dirs --url "https://raw.githubusercontent.com/Solipath/Solipath-Scripts/main/solipath_setup.bat" -o %HOMEDRIVE%%HOMEPATH%/solipath/solipath_setup.bat	
%HOMEDRIVE%%HOMEPATH%\solipath\solipath_setup.bat %*

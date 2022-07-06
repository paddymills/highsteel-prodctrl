$cnf_files = "\\hssieng\SNData\SimTrans\SAP Data Files\test"

Remove-Item $cnf_files\Issue_*
Remove-Item $cnf_files\backup\*
Remove-Item $cnf_files\archive\*
Copy-Item $cnf_files\_tests\* $cnf_files\
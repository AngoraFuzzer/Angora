cat $1 | sed  "s/.*dfs\$\(.*\)'.*/fun:\1=uninstrumented/"
cat $1 | sed  "s/.*dfs\$\(.*\)'.*/fun:\1=discard/"


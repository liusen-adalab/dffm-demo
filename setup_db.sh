# create db is not exist
set -e

# Read secret string
read_secret()
{
    # Disable echo.
    stty -echo

    # Set up trap to ensure echo is enabled before exiting if the script
    # is terminated while echo is disabled.
    trap 'stty echo' EXIT

    # Read secret.
    read "$@"

    # Enable echo.
    stty echo
    trap - EXIT

    # Print a newline because the newline entered by the user after
    # entering the passcode is not echoed. This ensures that the
    # next line of output begins at a new line.
    echo
}


read -p 'new database name: ' dbname

printf "password of user postgres: "
read_secret pw
export PGPASSWORD=$pw

STATUS=$(echo "SELECT 'CREATE DATABASE $dbname' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$dbname')\gexec" | sudo -u postgres psql)

if [ "$STATUS" != "CREATE DATABASE" ]; then
    echo "database $dbname already existed"
    exit 1
fi

for file in `find . -name up.sql` 
do 
  echo "importing $file"
  psql -U postgres -h localhost -d $dbname -p 5432 -q -f $file
done
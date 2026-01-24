# How to Run the End-to-End Test Script

### A mock data script that contains all the information needed to create an OSEDA project is required for the test script to work.


## Making the Mock Data Script
- You need to export a few specific variables in the mock data script that the test install script will use:
  - `PRIVATE_KEY`: the private ssh key generated for the GitHub account you are using to link to your OSEDA project
  - `PUBLIC_KEY`: the public ssh key generated for the github account you are using to link to you OSEDA project
  - `ADMIN_PASSWD`: password to the linux terminal you are running the script on
  - `USERNAME`: your Git username (assuming your Git and GitHub usernames are the same)
  - `EMAIL`: your Git email (assuming your Git and GitHub emails are the same)
- Create the `mock_data.sh` file:
```
touch mock_data.sh
{
  echo "read -r -d '' PRIVATE_KEY <<'KEY'"
  cat ~/.ssh/[PATH_TO_PRIVATE_KEY]
  echo "KEY"
  echo "export PRIVATE_KEY"
} > mock_data.sh
echo "export PUBLIC_KEY=\"$(cat ~/.ssh/[PATH_TO_PRIVATE_KEY])\"" >> mock_data.sh
echo "export ADMIN_PASSWD=\"[YOUR_PASSWORD]\"" >> mock_data.sh
echo "export USERNAME=\"[YOUR_GIT_USERNAME]\"" >> mock_data.sh
echo "export EMAIL=\"[YOUR_GIT_EMAIL]\"" >> mock_data.sh
```


- The file should look like this:
```
read -r -d '' PRIVATE_KEY <<'KEY'
-----BEGIN OPENSSH PRIVATE KEY-----
**********************************************************************
**********************************************************************
**********************************************************************
**********************************************************************
**********************************************************************
-----END OPENSSH PRIVATE KEY-----
KEY
export PRIVATE_KEY
export PUBLIC_KEY="ssh-ed25519 [YOUR_SSH_PUBLICKEY]"export ADMIN_PASSWD="12345"
export USERNAME="[YOUR_GIT_USERNAME]"
export EMAIL="[YOUR_GIT_EMAIL]"
```


## Running the Test Scipt
- Create the mock data script (using the commands above) and make sure it is in the same directory as the test script
- Give the test script executable permissions and run it using `source`
```
chmod +x ubuntu-test-intall.sh
source ubuntu-test-install.sh
```
- At the end of the output, it should print "Script succeeded" if all the dependencies were downloaded, a new OSEDA project was created, and `oseda check` passed.


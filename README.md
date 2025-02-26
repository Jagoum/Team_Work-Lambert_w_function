# Team Work Lambert w function

   ## INTRODUCTION
 This Project was made by the new team 4rust4goats so as to implement  the lambert_w function in rust.

 The lambert w funtion also call the omega fucntion is base on an equation that state that e^(w(x)) * w(x) = x , where the main objective is to find for the value of w . If you are trigger to know more about it I invite you  to look at this https://pmc.ncbi.nlm.nih.gov/articles/PMC8276860


  ## USAGE

  If you want to take a look on the project first start by clonning the repository by running the command 

    git clone https://github.com/Jagoum/Team_Work-Lambert_w_function.git

 Then to run the code you can either choose the short or the long way that is 

    cargo run --x=<argument>

OR
    
    cargo run -- --input=<argument>

   For the Dockerfile first start by building the image by running 

   

     docker pull ghcr.io/jagoum/team_lambert_watest:latest    
to pull the image 


    docker build -t <image_name> .

and run it using 

    docker run -it --rm --name<container_name> <image_name>


### CONTRIBUTION

 If you are interested in contributing you will have to  

      git pull https://github.com/Jagoum/Team_Work-Lambert_w_function.git



and finaly make a pull request so as for your contibution to be admit and merge to the main branch.

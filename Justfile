@default:
    just --list --unsorted

tool := "paper-age"
docker_user_repo := "mihaigalos"
docker_image_dockerhub := docker_user_repo + "/" + tool + ":latest"

build:
    sudo docker build --network=host -t {{ docker_image_dockerhub }} .

push:
    sudo docker push {{ docker_image_dockerhub }}

generate_pdf input_file +identities:
    cat {{ input_file }} | gzip | rage $(echo '{{ identities }}') -e -a > input_file
    docker run --rm -it -v $(pwd):/src docker.io/library/paper-age sh -c "cat input_file | paper-age --force --title=YubikeyIds --age-input --no-footer --identities='{{ identities }}'"


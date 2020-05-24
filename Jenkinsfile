// https://www.jenkins.io/doc/book/pipeline/docker/
pipeline {
    agent {
        docker { image 'piersfinlayson/build:latest' }
    }
    stages {
        stage('Test') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'github.packom', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                    sh '''
                        sudo apt update
                        sudo apt install -y default-jdk maven && \
                        export JAVA_HOME=/usr/lib/jvm/default-java && \
                        cd ~/builds && \
                        git clone https://@github.com/OpenAPITools/openapi-generator && \
                        cd ./openapi-generator && \
                        mvn clean install && \
                        cd ~/builds && \
                        git clone https://USERNAME:PASSWORD@github.com/packom/i2cbus-api && \
                        java -jar ~/builds/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate -i ./i2cbus-api/api/openapi.yaml -g rust-server -o ./i2cbus-api
                        cd i2cbus-api && \
                        echo "# i2cbus-api

i2cbus-api is an HTTP RESTful API designed to control an I2C bus.  This repo includes:
- An [API specification](https://github.com/packom/i2cbus-api/blob/master/api/openapi.yaml) in [OpenAPI format](https://github.com/OAI/OpenAPI-Specification/).
- Skeleton client and server implementations in [Rust](https://www.rust-lang.org/).

A fully-featured server implementation for Linux, in Rust, can be found at https://github.com/packom/i2cbus.

The text below was automatically generated by the openapi-generator.
" > /tmp/README.md && \
                        cat ./README.md >> /tmp/README.md && \
                        cp /tmp/README.md ./ && \
                        echo "[package]
name = \"i2cbus-api\"
version = "0.1.5"
authors = [\"Piers Finlayson <piers@packom.net>\"]
license = \"GPL-3.0-or-later\"
repository = \"https://github.com/packom/i2cbus-api\"
documentation = \"https://github.com/packom/i2cbus-api\"
homepage = \"https://github.com/packom/i2cbus-api\"
description = \"HTTP RESTful API and skeleton server/client implement for I2C bus control\"
readme = \"README.md\"
keywords = [\"i2c\",\"bus\",\"openapi\",\"swagger\",\"http\"]
categories = [\"api-bindings\",\"hardware-support\",\"network-programming\",\"embedded\",\"web-programming\"]
" > /tmp/Cargo.toml && \
                        tail -n +9 ./Cargo.toml >> /tmp/Cargo.toml && \
                        cp /tmp/Cargo.toml ./ && \
                        git status && \
                        git diff && \
                        git config --global user.email "piers@packom.net" && \
                        git config --global user.name "Piers Finlayson" && \
                        git add * && \
                        git commit -m "Checking in newly autogenerated version" && \
                        git push
                    '''
                }
            }
        }
    }
}

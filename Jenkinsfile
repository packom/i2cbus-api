// https://www.jenkins.io/doc/book/pipeline/docker/
pipeline {
    agent {
        docker { image 'piersfinlayson/openapi-gen-amd64:0.0.1' }
    }
    stages {
        stage('Clone') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'github.packom', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                    sh '''
                        cd ~/builds && \
                        git clone https://packom:$PASSWORD@github.com/packom/i2cbus-api && \
                        cd i2cbus-api && \
                        echo `awk '/^version / {print $3;}' Cargo.toml | sed 's/"//g'` > /tmp/old_version && \
                        echo "Old version is:" && \
                        cat /tmp/old_version
                    '''
                }
            }
        }
        stage('Auto-gen') {
            steps {
                sh '''
                    cd ~/builds && \
                    java -jar ~/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate --generate-alias-as-model -i ./i2cbus-api/api/openapi.yaml -g rust-server -o ./i2cbus-api
                    cd i2cbus-api && \
                    echo `awk '/^version / {print $3;}' Cargo.toml | sed 's/"//g'` > /tmp/new_version && \
                    echo "New version is:" && \
                    cat /tmp/new_version && \
                    NEWV=$(cat /tmp/new_version) && \
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
name = \\"i2cbus-api\\"
version = \\"$NEWV\\"
authors = [\\"Piers Finlayson <piers@packom.net>\\"]
edition = \\"2018\\"
license = \\"GPL-3.0-or-later\\"
repository = \\"https://github.com/packom/i2cbus-api\\"
documentation = \\"https://github.com/packom/i2cbus-api\\"
homepage = \\"https://github.com/packom/i2cbus-api\\"
description = \\"HTTP RESTful API and skeleton server/client implement for I2C bus control\\"
readme = \\"README.md\\"
keywords = [\\"i2c\\",\\"bus\\",\\"openapi\\",\\"swagger\\",\\"http\\"]
categories = [\\"api-bindings\\",\\"hardware-support\\",\\"network-programming\\",\\"embedded\\",\\"web-programming\\"]
" > /tmp/Cargo.toml && \
                    tail -n +9 ./Cargo.toml >> /tmp/Cargo.toml && \
                    cp /tmp/Cargo.toml ./ && \
                    find examples -name *.rs -print0 | xargs -0 sed -i 's/openapi_client/i2cbus_api/' && \
                    cat mods/lib.rs >> src/lib.rs && \
                    cat mods/client_mod.rs >> src/client/mod.rs && \
                    cat mods/client_remote.rs > src/client/remote.rs && \
                    git diff -- . ':(exclude)README.md' > /tmp/diff && \
                    cat /tmp/diff && \
                    echo `stat --printf="%s" /tmp/diff` > /tmp/diff_size && \
                    echo "Diff size is:" && \
                    cat /tmp/diff_size
                '''
            }
        }
        stage('Build') {
            steps {
                sh '''
                    cd ~/builds/i2cbus-api && \
                    cargo build
                '''
            }
        }
        stage('Test') {
            steps {
                sh '''
                    cd ~/builds/i2cbus-api && \
                    cargo test
                '''
            }
        }
        stage('Check in') {
            steps {
                sh '''
                    cd ~/builds/i2cbus-api && \
                    git config --global user.email "piers@packom.net" && \
                    git config --global user.name "Piers Finlayson" && \
                    git status && \
                    git diff && \
                    DIFF_SIZE=$(cat /tmp/diff_size) && \
                    if [ $DIFF_SIZE != 0 ] ; then git add -A && git commit -m "Checking in newly autogenerated version" && git push ; else echo "No changes to check in" ; fi
                '''
            }
        }
        stage('Publish') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'crates.packom', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                    // Note yank failure is OK because there'll be nothing to yank if we have a new version
                    sh '''
                        cd ~/builds/i2cbus-api && \
                        DIFF_SIZE=$(cat /tmp/diff_size) && \
                        OLDV=$(cat /tmp/old_version) && \
                        NEWV=$(cat /tmp/new_version) && \
if [ $DIFF_SIZE != 0 ]
then
    cargo publish --token $PASSWORD 
else
    echo "No changes to publish"
fi
                    '''
                }
            }
        }
    }
}

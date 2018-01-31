# -*- mode: ruby -*-
# vi: set ft=ruby :
Vagrant.configure("2") do |config|
  config.vm.box = "bento/fedora-27"
  config.vm.box_version = "201801.02.0"

  config.vm.provision 'shell', privileged: false, inline: <<-SHELL
    curl https://sh.rustup.rs -sSf > rustup.sh
    chmod +x ./rustup.sh
    ./rustup.sh -y
  SHELL

  config.vm.provision 'shell', inline: <<-SHELL
    dnf install -y openssl-devel
  SHELL
end

%global toby_confdir /etc/toby
%global unitdir /etc/systemd/system
%global bindir /usr/bin

Name: toby
Version: %{_version}
Release: 1
Summary: 🤖 Toby the friendly server bot
Group: System Environment/Daemons
License: MIT
Source0: toby.tar.gz
BuildArch: x86_64

%description
🤖 Toby the friendly server bot

%prep
tar -xvf %{SOURCE0}

%build
./configure --config-path /etc/toby \
            --log-path /var/log/toby \
            --runtime-path /var/lib/toby \
            --version %{_version}

cargo build --release

%install
[[ -d %{buildroot} ]] && rm -rf "%{buildroot}"

install -d -m 0755 %{buildroot}%{toby_confdir}
install -d -m 0755 %{buildroot}%{unitdir}
install -d -m 0755 %{buildroot}%{bindir}

cp %{_builddir}/conf/toby.toml %{buildroot}%{toby_confdir}/
cp %{_builddir}/conf/tokens.toml %{buildroot}%{toby_confdir}/
cp %{_builddir}/units/toby.service %{buildroot}%{unitdir}/
cp %{_builddir}/target/release/toby %{buildroot}%{bindir}/
cp %{_builddir}/target/release/tobyd %{buildroot}%{bindir}/

%post
systemctl --no-reload preset toby.service
mkdir -p %{toby_confdir}/conf.d

%clean
rm -rf %{_builddir}

%files
%defattr(-,root,root)
%dir %{toby_confdir}
%config(noreplace) %{toby_confdir}/toby.toml
%config(noreplace) %{toby_confdir}/tokens.toml
%{bindir}/toby
%{bindir}/tobyd
%{unitdir}/toby.service

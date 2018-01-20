%global toby_confdir /etc/toby
%global unitdir /etc/systemd/system
%global bindir /usr/bin

Name: toby
Version: 0.2.0
Release: 1
Summary: Toby the friendly server bot
Group: System Environment/Daemons
License: MIT
Source0: toby.tar.gz
BuildArch: x86_64

%description
Toby the friendly server bot

%prep
tar -xvf %{SOURCE0}

%build
cargo build --release

%install
[[ -d %{buildroot} ]] && rm -rf "%{buildroot}"

install -d -m 0755 %{buildroot}%{toby_confdir}
install -d -m 0755 %{buildroot}%{toby_confdir}/conf.d
install -d -m 0755 %{buildroot}%{unitdir}
install -d -m 0755 %{buildroot}%{bindir}

cp %{_builddir}/conf/toby.toml %{buildroot}%{toby_confdir}/
cp %{_builddir}/units/toby-server.service %{buildroot}%{unitdir}/
cp %{_builddir}/units/toby-worker.service %{buildroot}%{unitdir}/
cp %{_builddir}/target/release/toby %{buildroot}%{bindir}/

%post
systemctl --no-reload preset toby-server.service
systemctl --no-reload preset toby-worker.service

mkdir -p %{toby_confdir}

%clean
rm -rf %{_builddir}

%files
%defattr(-,root,root)
%dir %{toby_confdir}
%dir %{toby_confdir}
%config(noreplace) %{toby_confdir}/toby.toml
%{bindir}/toby
%{unitdir}/toby-server.service
%{unitdir}/toby-worker.service

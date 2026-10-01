Name:           herdrscreen
Version:        0.1.0
Release:        1%{?dist}
Summary:        Terminal workspace manager for AI coding agents (GNU screen edition)
License:        Apache-2.0
URL:            https://github.com/sadsfae/herdrscreen
Source0:        herdrscreen
Source1:        LICENSE
Source2:        NOTICE
Source3:        README.md
Source4:        CHANGELOG.md
BuildArch:      x86_64
# statically linked musl binary; tools it spawns at runtime:
Requires:       openssh-clients, git, curl

%description
herdrscreen is a GNU screen edition hard fork of Herdr: a terminal workspace
manager for AI coding agents. It keeps Herdr's engine and defaults to GNU
screen keybindings (prefix ctrl+a, ctrl+a ctrl+a toggles the last focused
tab). The binary is statically linked and self contained; updates ship as
packages, never self-update.

herdrscreen is distributed under the Apache License 2.0. It is a fork of
Herdr (https://github.com/herdrdev/herdr), Copyright (C) 2024-2026 the Herdr
Project contributors. See LICENSE and NOTICE.

%prep

%build

%install
install -Dm755 %{SOURCE0} %{buildroot}%{_bindir}/herdrscreen
install -Dm644 %{SOURCE1} %{buildroot}%{_licensedir}/herdrscreen/LICENSE
install -Dm644 %{SOURCE2} %{buildroot}%{_licensedir}/herdrscreen/NOTICE
install -Dm644 %{SOURCE3} %{buildroot}%{_docdir}/herdrscreen/README.md
install -Dm644 %{SOURCE4} %{buildroot}%{_docdir}/herdrscreen/CHANGELOG.md

%files
%{_bindir}/herdrscreen
%license %{_licensedir}/herdrscreen/LICENSE
%license %{_licensedir}/herdrscreen/NOTICE
%doc %{_docdir}/herdrscreen/README.md
%doc %{_docdir}/herdrscreen/CHANGELOG.md

%changelog
* Thu Oct 01 2026 sadsfae <sadsfae@users.noreply.github.com> - 0.1.0-1
- Initial herdrscreen package (fork of Herdr 0.9.3, GNU screen defaults)

#!/usr/bin/env bash

set -e 

# 5. Написать командный файл для автоматизации создания резервной копии базы данных таким образом, 
#   чтобы полученный файл резервной копии содержал в своём имени дату и время создания резервной копии, 
#   а также был автоматически заархивирован (использовать архиватор 7zip).

# NOTE. 
# I'll use tar and bash instead

msg() {
  echo >&2 -e "${1-}"
}

die() {
  local msg=$1
  local code=${2-1} # default exit status 1
  msg "$msg"
  exit "$code"
}

have() {
    type "$1" &> /dev/null || die "$1 is not installed"
}

have mysqldump
if [ -z "$1" ] 
  then
  die "Expected database name"
fi

CURTIME=$(date +'%Y_%m_%d_%H_%M_%S')
NAME="$1"

BACKUP_NAME="${NAME}_${CURTIME}"

mysqldump ${NAME} > "${BACKUP_NAME}"
tar -czf "${BACKUP_NAME}.tar.gz" "${BACKUP_NAME}"
rm "${BACKUP_NAME}"


ssh-keygen -t ed25519 -C "danila.sar@yandex.ru" -f ~/.ssh/tutors-deploy
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/tutors-deploy

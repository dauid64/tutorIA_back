# TutorIA back-end

API REST utilizada na aplicação TutorIA

## Build da Imagem Docker

Para executar sem ter o rust, basta ter o docker e fazer o build da imagem conforme comando abaixo. Recomenda-se usar conforme abaixo, a imagem Dockerfile.dev para desenvolvimento, pois o build é mais rápido. Para produção, ou para máxima performance do executável gerado, usar o build sem o -f Docierfile.dev. Neste último caso, o build pode levar de 10 a 20 minutos dependendo da máquina.

Porém, antes de executar o build da imagem, você precisa ajustar as váriaveis da URL com o banco, que estão localizadas em `.cargo/config.toml` e `src/_dev_utils/dev_db.rs`. Basta alterar localhost para db.

```bash
docker build -f Dockerfile.dev -t dauid64/tutoria1.0 .
```
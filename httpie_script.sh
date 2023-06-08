#!/bin/bash

# Verificar o status de saúde
http GET http://localhost:8000/health_check

# Obter uma tarefa pelo ID
http GET http://localhost:8000/task/1

# Listar todas as tarefas
http GET http://localhost:8000/tasks

# Criar uma nova tarefa
http POST http://localhost:8000/task name="Nova tarefa" completed:=false

# Atualizar uma tarefa existente pelo ID
http PUT http://localhost:8000/task/{id} name="Tarefa atualizada" completed:=true

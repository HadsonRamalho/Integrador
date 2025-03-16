package com.integrador.MaqExpress.controller;

import java.util.Optional;
import java.util.UUID;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;

import com.integrador.MaqExpress.model.UserModel;
import com.integrador.MaqExpress.repository.UserRepository;

import jakarta.validation.Valid;

public class UserController {
    @Autowired
    private UserRepository userRepository;
    @PostMapping("/cadastra_usuario")  
    public ResponseEntity<?> createUser(@RequestBody @Valid UserModel usuario){
        try {
            userRepository.save(usuario);//Salvando o usuario no banco de dados, usando o método save do Jpa
            
        } catch (Exception e) {
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Ocorreu um erro inesperado" + e.getMessage());
        }
        return ResponseEntity.ok("Usuario cadastrado com sucesso");
    }
    @GetMapping("/busca_usuario_id")
    public ResponseEntity<?> buscaUsuarioId(@RequestBody UUID id){
        try {
           Optional<UserModel> user = userRepository.findById(id);
           return ResponseEntity.ok(user.get().getIdUsuario());
        } catch (Exception e) {
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Ocorreu um erro inesperado" + e.getMessage());
        }
    }
    @GetMapping("/busca_email_usuario")
    public ResponseEntity<?> buscaEmailUsuario(@RequestBody UUID id){
        try{
            Optional<String> email = userRepository.findEmailById(id);
            return ResponseEntity.ok(email);
        }catch(Exception e){
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Ocorreu um erro inesperado" + e.getMessage());

        }
    }
}

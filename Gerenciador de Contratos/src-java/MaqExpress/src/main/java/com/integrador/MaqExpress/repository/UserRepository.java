package com.integrador.MaqExpress.repository;

import java.util.Optional;
import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import com.integrador.MaqExpress.model.UserModel;

import jakarta.transaction.Transactional;

public interface UserRepository extends JpaRepository<UserModel, UUID> {
    @Query("SELECT u.email FROM UserModel u WHERE u.id = :id")
    Optional<String> findEmailById(@Param("id") UUID id);
    @Query("SELECT u.id FROM UserModel u WHERE u.email = :email")
    Optional<UUID> findIdByEmail(@Param("email") String email);
    @Query("SELECT u.id FROM UserModel u WHERE u.email = :email AND origemconta = 'Google'")
    Optional<UUID> findIdByEmailAndOrigemConta(@Param("email") String email);
    @Query("SELECT u.senha FROM UserModel u WHERE u.email = :email")
    Optional<String> findSenhaByEmail(@Param("email") String email);
    @Modifying
    @Transactional
    @Query("UPDATE UserModel u SET u.senha = :novaSenha WHERE u.email = :email")
    void updateSenhaByEmail(@Param("email") String email, @Param("novaSenha") String novaSenha);
    Optional<UserModel> findByDocumento(String documento);

    
    
}

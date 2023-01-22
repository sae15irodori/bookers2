class UsersController < ApplicationController
  def show
    @user = User.find(params[:id])#特定ﾕｰｻﾞｰidのﾚｺｰﾄﾞ取得部分ﾃﾝﾌﾟﾚに渡す
    @books = @user.books
    @book = Book.new#部分ﾃﾝﾌﾟﾚに渡す
  end

  def index
    @users = User.all
    @book = Book.new
    @user = current_user#部分テンプレへ渡す
  end

  def edit
    @user = User.find(params[:id])
  end

  def update
    @user = User.find(params[:id])
    if @user.update(user_params)#更新時条件に当てはまったら
      flash[:notice]  = "You have updated user successfully."
      redirect_to user_path(current_user.id)
    else                        #当てはまらなければ
      render :edit
    end
  end


    private
    def user_params
      params.require(:user).permit(:name, :introduction, :profile_image)
    end

end
